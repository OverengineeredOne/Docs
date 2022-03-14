# Preface

Before we get started I wanted to give you a little background on Overengineered and some of the 
original problems that set out to solve. This will help set the direction and tone of the
text.

First, why Overengineered? We'll it embodies the spirit of *"I'll spend four hours automating a 
task that takes fifteen minutes to complete."* In the end the most important thing is that we 
learned something and are having fun. 

Now what defines our platform and vision is a little more complicated. It mainly derives from a
few different things inspired by my career.

I've worked within a large tech company for the last ten years solving problems of various scale. 
My strength has always been solving problems that drastically increase operational and development
efficiency across the company. Within a product company we're often highly well tuned engines.
Outside of software engineering, we're data science, data engineering, production engineering, 
user experience research, user research, language design all working to identify and tackle 
problems that essentially saves the most amount of cognitive or physical work. This essentially 
allows entire organizations to maintain minimal budget increases year over year as the number of 
end users within products scale to billions of users and trillions of transactions.

This has always been a problem with my personal projects when I try to have direct impact in my 
life. I have accumulated a large number of unfinished products to tackle some minor problems.
But often the real world problems I'd like to tackle that will have a substantial impact to my 
household budgeting, or optimizing my time spent on daily errands, or chores. The scope is solving 
some of these problems is substantial and it's unfortunate that there is not a way I can hack a 
solution together and get a product distributed within a weekend. I end up burning a significant 
amount of calories planning, budgeting materials and costs, setting up my infrastructure, writing 
the glue code between services or devices.

More often than not I end up in a debate with myself if I should use a cloud service or spend the 
extra effort to build or find my own solution.

The other driving factors that influence Overengineered are the general populations sentiment 
towards technology companies and tech. To me, the driving factor of this seems to be trust. People
want their data secure, protected, private, and available. Blockchain attempts to solve this 
problem. It is a great tool and has a purpose, but is generally to expensive and slow to be used
everywhere.

So, I asked myself, why not bring the cloud infrastructure home? Make it as easy as Installing an
operating system. Adding a new computer or device is as easy as scanning a QR code. This is 
prefect for running facial recognition on security cameras within the house. Or tracking 
individuals throughout the house to apply individualized home automation and contextual 
interaction with IoT devices. This is not the type of data my wife feels safe traveling outside a 
private closed network, where we have tight privacy policies based on who is in the video.

The next question this leads to is, how do we easily distribute apps that would use a private
cloud? The first thing that comes to mind is Helm, but that requires someone involved who has 
knowledge on how Kubernetes works and how to set up these applications. Why not provide a data 
service and abstraction that can download data and manage schemas from public registries.

Progressive web apps can connect to any hosted data service and request for an install of the 
required data schemas for the app to function. This public and transparent data model approach has 
the added benefit of creating a large data graph. Other abstractions can be made on top to provide 
a federated learning platform. 

Not only could this enrich the value of user data to something beyond whats easily attainable 
today. But gives the true owner of the data power and the ability to monetize their own anonymized
data in a safe and secure manor.

This defines our north stars or what we aim to achieve. This is a long term vision and much of 
this will inevitably change. Through the text You may find developer notes describing an approach 
to solving some of developer experience challenges. If your interested in contributing 
towards these areas. Feel free to open an issue to kick off the discussion.

-- Sean O'Rourke
