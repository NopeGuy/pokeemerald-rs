IPMX Presentation

Hello everyone and welcome to this presentation about IPMX.
This shall be a small mostly introductory piece to show what it is, why it is important in our field and what it enables people and products to do.
Due to the nature of what I’ll be going to talk about today, this presentation shall be of a more theoretical nature instead of a more practical one like the one that Francisco has made about docker.

So firstly, what better way to start then...
What is IPMX?
Lately we've all been hearing a more about IPMX due to the Multiviewer's integration of the specification and it’s certification, but what is it?
Firstly, IPMX is a suite of open standards and specifications (like SMPTE ST 2110, AMWA NMOS, and various VSF Technical Recommendations [show concrete examples]) that were created so systems can communicate interoperably to trade audio and video between them and their relevant metadata.
The videos transmitted under the umbrella of ipmx can both be raw for lossless true to the source video or compressed like (for instance JPEG-XS or HEVC) for less bandwidth usage. This flexibility means you can choose between great quality for production environments or efficient compression when bandwidth is limited, for example when streaming to homes.

What are these standards and specifications?
To understand IPMX, we need to look at the building blocks it's made from and the organizations behind them.
The Foundation: SMPTE ST 2110
SMPTE ST 2110 is the backbone of IPMX. It's a suite of standards that defines how to send uncompressed video, audio, and data over IP networks. Think of it as the rules for media data to be traded between devices, when the digital media creation and transport methods came to replace analog methods like SDI [show example of both digital and analog media handling].
The Control Layer: AMWA NMOS
AMWA (or Advanced Media Workflow Association) created the NMOS (Networked Media Open Specifications) suite to solve a crucial problem: how do you discover, connect, and manage all these streams in a large facility?
For that, NMOS includes several specifications that explain how to discover,register, manage connections and much more between stream producers and consumers.
An easier way of thinking about NMOS would be to imagine it as a middle man who is advertising every node, device, sender and receiver to make sure all your devices can find each other and talk properly with help of the control center advertising them useful information.
The Video Services Forum (VSF)
VSF is another key organization that's contributed essential specifications to IPMX. They're the ones behind the Technical Recommendations (TRs) that fill in gaps and provide practical implementation guidance. Answering questions like "how much bandwidth do I really need?" and "how do I make sure my timing is perfect?"

So, ...
What's the use for IPMX?
IPMX basically takes all of these proven standards and packages them together with certification requirements, ensuring that when a device says it's "IPMX certified," you know exactly what it can do and that it'll work with other certified devices.
Here's the simple way to think about it:
    • SMPTE ST 2110 = The language (how video and audio are packaged and sent) 
    • AMWA NMOS = The conversation protocol (how devices find each other and communicate) 
    • VSF TRs = The practical guide (how to actually make it all work in your case) 
Due to all those efforts, you can know that when a product is IPMX certified you can guarantee some things like:
Timing Control
Through the use of the PTP (Precision Time Protocol - IEEE 1588) to keep everything synchronized down to the nanosecond. This is crucial when you're mixing sources from different parts of your facility or even different locations. So you don’t have to deal with frame sync issues or audio/video drift.
Interoperability and Integration
Leveling the playing field and facilitating integration by making sure all devices can communicate and operate with the same rules so no more setup is required than simply connecting a sender and a receiver.
And Easy Setup
Making it so that you don’t have to waste a lot of time configuring everything manually and piece by piece. With this, you can be sure your system can be up and running as soon as devices are connected and powered on. Kind of like how nowadays many smart devices like our TV’s and phones can connect with each other as soon as they are on the same network. 
And much more!
Real-World Example
Imagine you're building a new production facility. With IPMX:
    1. Your cameras output ST 2110 streams that are automatically discovered by NMOS 
    2. Your router uses IS-05 to connect any source to any destination 
    3. Your multiviewer discovers all available sources via IS-04 and can display them without manual configuration 
    4. Your monitoring system uses IS-09 to control camera settings across the facility 
    5. All devices stay perfectly synchronized via PTP 
    6. When you add new equipment, it announces itself and integrates automatically 
Compare this to traditional SDI where you'd need to manually cable everything, create configuration files for every device, and carefully plan your routing capacity in advance, where here you just need to be sure you have enough bandwidth for every stream you’ll be sending out.
Why IPMX Matters Now (in this time point specifically)
The industry is at an inflection point. The old infrastructure is aging, and the way things were done before is getting outdated due to new demands on our systems like more demanding formats (4K, 8K, HDR) making us have the need to rethink about the technologies we use.
IPMX for that purpose, provides a proven standardized path forward. This initiative ensures everyone is together under one certification umbrella, making adoption easier and ensuring a better development between everyone in our industry in the future.

Thank you all for coming and listening to my presentation! Hope you all enjoyed and if you have any questions feel free to ask!

References and documents in case you want to learn more:
- VSF TR 10 link
- IPMX link
- AMWA NMOS link
- SMPTE 2110 specification
