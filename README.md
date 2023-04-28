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
"to":"userID"
"message":"message"
}

- Header must include sender UserID
- Relays message to RabbitMQ queue

---
**GET:**

*/message/username?to="id"start="index"&stop="index"*  
returns any number messages from user depending on query sorted by time and date

- to = UserId

- start = index

- stop = index
- Json: {
"from": "userID",
"to":"userID",
"message":"message",
"date":"20230101:00:00:00"}

*/health_check*  
returns 200 to check if server is up

---
