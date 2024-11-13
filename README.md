# API CON KUBERNETES

primeramente instalamos kubectl utilizando cmd como administrador

![image](https://github.com/user-attachments/assets/37fbf193-ac17-4891-81de-53ee1b1cf81b)

verificamos la instalación

![image](https://github.com/user-attachments/assets/af34c533-0f1c-4cb2-a496-bd4063edd9b0)

ahora desde el powershell como administrador descargamos minikube

![image](https://github.com/user-attachments/assets/e9cd3989-3e14-49c4-9413-c12283a02e69)

ahora lo agregamos al PATH

![image](https://github.com/user-attachments/assets/c665d761-662f-459b-be36-ac89c6f9c42f)

y revisamos que este instalado

![image](https://github.com/user-attachments/assets/37d0858f-d239-48f3-b6f6-ea0e24b2316f)

iniciamos minikube utilizando Virtual Box

![image](https://github.com/user-attachments/assets/bbf7c771-a181-469a-9ba3-d139d665afde)

verificamos el status de minikube

![image](https://github.com/user-attachments/assets/5df22f99-c1d2-4f84-8708-66a3c4f2eeb0)

ahora nos dirijimos a la carpeta donde estamos trabajandoy creamos un archivo .yaml para hacer el despliegue de un pod de mysql

```
apiVersion: apps/v1
kind: Deployment
metadata:
  name: mysql
spec:
  selector:
    matchLabels:
      app: mysql
  template:
    metadata:
      labels:
        app: mysql
    spec:
      containers:
      - name: mysql
        image: mysql:8
        env:
        - name: MYSQL_ROOT_PASSWORD
          value: "OMjaa1211"
        - name: MYSQL_DATABASE
          value: "actividades_extraescolares"
        ports:
        - containerPort: 3306
```

ahora utilizamos el comando ```kubectl apply -f``` para desplegar nuestro MySQL

![image](https://github.com/user-attachments/assets/e1008ae2-197d-49dd-8eab-ea1d73861471)

utilizamos el comando ```kubectl get pods``` para comprobar que esta corriendo

![image](https://github.com/user-attachments/assets/bf68ab3d-292c-4687-a451-4c875d4f0d7c)

ahora generaremos otro documento en formato yaml para configurar un servicio utilizando ClusterIP

```
apiVersion: v1
kind: Service
metadata:
  name: mysql-service
spec:
  ports:
  - port: 3306
    targetPort: 3306
  selector:
    app: mysql
  type: ClusterIP
```
posteriormente con el comando ```kubectl apply -f``` lo pondremos en marcha, despues con ```kubectl get svc``` confirmamos que este corriendo

![image](https://github.com/user-attachments/assets/ebb782c4-e350-4643-929b-91a927abc087)

utilizando el comando ```kubectl exec -it mysql-75f78db97d-c268g -- mysql -u root -p``` y posteriormente podemos entrar a nuestra base de datos y crearemos la tabla

![image](https://github.com/user-attachments/assets/9fe01cde-11b6-46a3-b3e4-cd4a260693e3)
![image](https://github.com/user-attachments/assets/8c032e76-ac38-47a1-8b5f-de571a206d5c)

ahora creamos un archivo .yaml para hacer el despliegue de nuestra API
```
apiVersion: apps/v1
kind: Deployment
metadata:
  name: api-actividades
spec:
  replicas: 5
  selector:
    matchLabels:
      app: api
  template:
    metadata:
      labels:
        app: api
    spec:
      containers:
      - name: api
        image: javierescobar12/actividades-api-rust
        ports:
        - containerPort: 5050
```

ahora utilizamos el comando ```kubectl apply -f``` para desplegar nuestra API

![image](https://github.com/user-attachments/assets/cda3d489-3cdf-40d1-9e2a-2677970f88fc)

utilizando el comando ```kubectl get pods``` podemos observar que se comenzará a crear el container y posteriormente aparecerán corriendo

![image](https://github.com/user-attachments/assets/ad96e472-5c4a-4f2b-b7ad-446f9e992b24)
![image](https://github.com/user-attachments/assets/eef32c32-08f7-4fe6-83fc-89d181c0e91e)

ahora crearemos un .yaml para crear nuestro servicio de manera declarativa
```
apiVersion: v1
kind: Service
metadata:
  name: api-service
spec:
  type: NodePort
  selector:
    app: api
  ports:
    - port: 5050
      targetPort: 5050
      nodePort: 30007
```
ahora con apply lanzaremos nuestro servicio dentro de minikube en el puerto 30007

![image](https://github.com/user-attachments/assets/f4db567d-3122-4ea6-9595-fb9ae7e63115)

utilizando el comando ```minikube ip``` obtendremos la ip que tiene nuestro minikube a lo cual podremos hacer peticiones de nuestra api por el puerto 30007

![image](https://github.com/user-attachments/assets/32ead73c-cbec-47c6-b585-c9bd1707bfd7)
![image](https://github.com/user-attachments/assets/0ba5f0a4-9b4c-4035-b2be-bd99bf05c693)

ahora habilitaremos ingress para que usar nuestra api atraves de un dominio

![image](https://github.com/user-attachments/assets/e1944857-ae35-4c57-81fb-0b150daf33ac)

creamos un archivo yaml donde definiremos los diferentes pads que tendra nuestra api
```
apiVersion: networking.k8s.io/v1
kind: Ingress
metadata:
  name: actividades-api-ingress
spec:
  rules:
  - host: actividades-api.com
    http:
      paths:
      - path: /insertar-actividad  
        pathType: Prefix
        backend:
          service:
            name: api-service  
            port:
              number: 5050  
      - path: /actividad  
        pathType: Prefix
        backend:
          service:
            name: api-service  
            port:
              number: 5050       
      - path: /actividad-id
        pathType: Prefix
        backend:
          service:
            name: api-service
            port:
              number: 5050 
      - path: /borrar-actividad
        pathType: Prefix
        backend:
          service:
            name: api-service
            port:
              number: 5050 
```
ahora utilizamos apply para crear nuestro ingresss

![image](https://github.com/user-attachments/assets/41934fc4-97b2-425f-a052-2513c09aa8df)

ahora modificaremos los hosts en la computadora para que se pueda identificar el dominio, primero obtendremos la ip correspondiente a ese ingress

![image](https://github.com/user-attachments/assets/02bde405-58c5-4c7d-9344-25d55ff4cdc6)

posteriormente en la carpeta drivers\etc modificaremos el archivo hosts y agregaremos la ip y el dominio de nuestro ingress

![image](https://github.com/user-attachments/assets/b8ff227c-0dc9-4d52-930f-c2476de209e3)
![image](https://github.com/user-attachments/assets/f1529bb9-5c2c-454e-8fe1-b4d0b0d44dcb)

ahora podemos entrar a nuestra api a traves del dominio

![image](https://github.com/user-attachments/assets/ba094b34-30d0-42a9-8ed7-f93e10da56c9)

ahora utilizaremos Postman para hacer diferentes peticiones

![image](https://github.com/user-attachments/assets/94b288fb-3520-4bbf-9eea-b3badc9a7f4e)
![image](https://github.com/user-attachments/assets/3ee70b87-d979-4c6a-a609-5fedf99bb36a)
![image](https://github.com/user-attachments/assets/dad7de91-f726-4172-860c-a4801156162c)
![image](https://github.com/user-attachments/assets/898ffea9-2bbd-4365-a6f1-2d5729ef2069)
![image](https://github.com/user-attachments/assets/97a03d7f-6ba1-4c48-9951-b571ed4d4cdc)




