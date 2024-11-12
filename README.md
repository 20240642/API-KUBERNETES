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

iniciamos minikube utilizando el driver de docker

![image](https://github.com/user-attachments/assets/a4e90a25-1763-49c4-ada1-e8ffd2813ca9)

verificamos el status de minikube

![image](https://github.com/user-attachments/assets/5df22f99-c1d2-4f84-8708-66a3c4f2eeb0)

ahora nos dirijimos a la carpeta donde estamos trabajando y creamos un archivo .yaml para hacer el despliegue de nuestra API
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

![image](https://github.com/user-attachments/assets/8ddd5e30-e1c4-452a-bc43-309ffcd0aca0)

utilizando el comando ```kubectl get pods``` podemos observar que se comenzará a crear el container y posteriormente aparecerán corriendo

![image](https://github.com/user-attachments/assets/5f44f73a-fdda-48f9-9e75-1cf11dd22bd9)
![image](https://github.com/user-attachments/assets/494c6036-2130-4f43-a594-90f91d406a54)

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

![image](https://github.com/user-attachments/assets/4cbce830-ab1c-4236-b7ad-265387735b88)

debido a que estamos utilizando el driver de docker, la red de nuestro minikube no se comparte con la de nuestro equipo, asiq ue expondremos el servicio utiliando el comando minikube service

![image](https://github.com/user-attachments/assets/3cc5dc7a-d8be-4939-88d1-23ef1451017f)

esto nos dara una ip con la cual podemos probar nuestra API

![image](https://github.com/user-attachments/assets/f2727691-9ac1-4b3a-8a87-b8cdf273d355)

![image](https://github.com/user-attachments/assets/643bf114-52e6-4996-acc5-1870e12a690b)

![image](https://github.com/user-attachments/assets/fb2dc126-d71c-430b-a4be-f102ecd0a979)

![image](https://github.com/user-attachments/assets/2e9798cb-4920-4146-8cf3-72b559cdad4c)

tambien podemos comprabar el funcinamiento en un pod especifico utilizando el comando ```kubectl proxy```

![image](https://github.com/user-attachments/assets/dd7aed26-9426-42b4-b986-e6c5a7947cee)




