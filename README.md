## Post service

ITHS Webservices and integrations laboration

Service to handle messages.
Saves to MongoDB.

### Endpoints:

---
**POST:** 

*/newmessage*
- Except Json in the format;   
{  
"to": "userID",  
"message": "message"  
}  

- Header must include sender userID  .
_example header: userID 123_
- Will return a location header with the url to the message.

---
**GET:**

*/health_check*  
returns 200 to check if server is up


*/message/"id"*
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

*/chat?to="user_id"*

* Header must include sender userID.
* Returns a Json with all messages from the userID in header to the user_id in the query.

_/message?to="user_id"&fromMessageId="message_id"&numberOfMessages="quantity"_

* Header must include sender userID.
* Returns any number messages from userID in header to user_id greater then the message_id in the query.
example:  
say i have a header with userID 1 and make a get request as follows:  
  _/message?to=2&fromMessageId=645294e0863e17b5543a0384&numberOfMessages=3_  
This will return 3 json messages that was made from userID 1 to userID 2 newer than the message in the message_id query. 

---
**Settings**

#### This environment variable must be set with the MongoDB connections string.
ROCKET_DATABASES='{postservice={url="mongodb://<MONGOURL:27017>"}}'  
Replace "MONGOURL" with the connection string to your database.  
The app will save to a database called postservice and a collection called messages

---

**Alternative settings**

ROCKET_PORT="8000" (Default)  
ROCKET_ADRESS="0.0.0.0" (Default)  
ROCKET_LOG_LEVEL="normal" (Default) (normal/debug/critical)  



