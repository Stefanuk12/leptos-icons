use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M8 6v6" ></ path > < path d = "M15 6v6" ></ path > < path d = "M2 12h19.6" ></ path > < path d = "M18 18h3s.5-1.7.8-2.8c.1-.4.2-.8.2-1.2 0-.4-.1-.8-.2-1.2l-1.4-5C20.1 6.8 19.1 6 18 6H4a2 2 0 0 0-2 2v10h3" ></ path > < circle cy = "18" r = "2" cx = "7" ></ circle > < path d = "M9 18h5" ></ path > < circle r = "2" cx = "16" cy = "18" ></ circle > < / > } } pub const LUCIDE_BUS : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("width" , "24") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round")] } ;