use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M5.5 20H8" ></ path > < path d = "M17 9h.01" ></ path > < rect width = "10" y = "4" height = "16" rx = "2" x = "12" ></ rect > < path d = "M8 6H4a2 2 0 0 0-2 2v6a2 2 0 0 0 2 2h4" ></ path > < circle cy = "15" r = "1" cx = "17" ></ circle > < / > } } pub const LucideMonitorSpeaker : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("stroke-width" , "2")] } ;