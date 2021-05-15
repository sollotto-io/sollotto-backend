
  const  windowclose = () => {
        const openedWindow = window.open(
            "https://google.com",
            "Google Search",
            "width=800,height=600,resizable,scrollbars"
          );
          
          // check if the window is in opened or closed state
          console.log(openedWindow.closed); // false
    }

 