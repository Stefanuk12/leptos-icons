use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle r = "5.5" cy = "15.5" cx = "7.5" ></ circle > < path d = "m21 2-9.6 9.6" ></ path > < path d = "m15.5 7.5 3 3L22 7l-3-3" ></ path > < / > } } pub const LUCIDE_KEY : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("height" , "24") , ("width" , "24") , ("stroke-width" , "2")] } ;