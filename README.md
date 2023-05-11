## Post service

ITHS Webservices and integrations laboration

Service to handle messages.
Saves to MongoDB.

### Endpoints:

---
**POST:**

*/posts*

- Except Json in the format;   
  {  
  "to": "userID",  
  "message": "message"  
  }

- Header must include sender userID .
  _example header: userID 123_
- Will return a location header with the url to the message.
- Sends the message to RabbitMQ queue called "messages"

---

**DELETE:**

*/posts/"id"*

* Deletes requested message


---
**GET:**

*/health_check*  
returns 200 to check if server is up

*/posts/"id"*

* Return a Json if the requested id:
* {  
  "_id": {  
  "$oid": "64535b421665a078fe677c5c"  
  },  
  "from": "123",  
  "to": "3232",  
  "message": "Hello there",  
  "date": "2023-05-04T07:14:10.392574957Z"  
  }

*/posts/conversations*

* Header must include sender userID.
* Returns all userIDs that the user in Header have conversations with.

*/posts?to="user_id"*

* Header must include sender userID.
* Returns all messages from the userID in header to the user_id in the query as Json sorted by date.

**Optional query parameters for chat**

* _limit="number"_  
  Limits the number of messages returned
* _messageId="the id of a message"_  
  Sets the starting point where to return messages. Any messages with an id greater than the set id will be returned.

**Example:**  
say i have a header with userID 1 and make a get request as follows:  
_/posts?to=2&messageId=645294e0863e17b5543a0384&limit=3_  
This will return 3 json messages that was made from userID 1 to userID 2 newer than the message in the messageId query.

---
**Settings**

#### This environment variable must be set with the MongoDB connections string.

ROCKET_DATABASES='{postservice={url="mongodb://<MONGOURL:27017>"}}'  
Replace "MONGOURL" with the connection string to your database.  
The app will save to a database called postservice and a collection called messages


ROCKET_RABBIT_HOST="amqp://localhost:5672" (Default)  
This sets the connections string to RabbitMQ  
The application sends a Json to a queue called "messages" each time a POST request is made on /posts

---

**Alternative settings**

ROCKET_PORT="8000" (Default)  
ROCKET_ADRESS="0.0.0.0" (Default)  
ROCKET_LOG_LEVEL="normal" (Default) (normal/debug/critical)  



