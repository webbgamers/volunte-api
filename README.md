# volunte-api
Knight Hacks '23 project.
Making it easier for event organizers to manage volunteers. Making it easier for volunteers to find events and sign up for jobs they want.

# Endpoints
All request and response data is JSON.  

## \[POST\] /register
Registers a new user and returns user info.  
### Arguments
`email` - Email for the user.  
`name` - Name for the user.  
`password` - Password for the user.  
### Response
`id` - Hexadecimal ID of the user.  

## \[GET\] /login
Gets user data if credentials match known user.  
### Arguments
`email` - Email of the user.  
`password` - Password of the user.  
### Response
`id` - Hexadecimal ID of the user.  
`email` - Email of the user.  
`name` - Name of the user.  
`password` - Password of the user.  

## \[GET\] /user
Gets user data from hexadecimal ID.  
### Arguments
`id` - Hexadecimal ID of the user.  
### Response
`id` - Hexadecimal ID of the user.  
`email` - Email of the user.  
`name` - Name of the user.  
`password` - Password of the user.  

## \[GET\] /event
Gets event data from hexadecimal ID.  
### Arguments
`id` - Hexadecimal ID of the event.  
### Response
`id` - Hexadecimal ID of the event.  
`name` - Name of the event.  
`address` - Physical address of the event.  
`owner` - Owner object.  
`timeslots` - Array of Timeslot objects.  

Owner - Creator of the event.  
+ `id` - Hexadecimal ID of the owning user.  
+ `name` - Name of the owning user.
  
Timeslot - Defined time period where volunteers are needed.  
+ `start` - Start time of time slot in RFC-3339 encoding.  
+ `end` - End time of time slot on RFC-3339 encoding.  
+ `volunteers` - Array of Volunteer objects.  
+ `requests` - Array of Request objects.
  
+ Volunteer - A user who has been approved to volunteer for a specific role and time slot.  
  + `id` - Hexadecimal ID of the volunteering user.  
  + `name` - Name of the volunteering user.  
  + `role` - Role of the volunteering user.

+ Request - A user who has requested to volunteer for specified roles for a time slot.  
  + `id` - Hexadecimal ID of the requesting user.  
  + `name` - Name of the requesting user.  
  + `roles` - Array of Strings for each specified role.  
    
