use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m12.99 6.74 1.93 3.44" ></ path > < path d = "M19.136 12a10 10 0 0 1-14.271 0" ></ path > < path d = "m21 21-2.16-3.84" ></ path > < path d = "m3 21 8.02-14.26" ></ path > < circle r = "2" cx = "12" cy = "5" ></ circle > < / > } } pub const LUCIDE_DRAFTING_COMPASS : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke" , "currentColor") , ("height" , "24") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke-width" , "2")] } ;