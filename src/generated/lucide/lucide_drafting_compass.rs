use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m12.99 6.74 1.93 3.44" ></ path > < path d = "M19.136 12a10 10 0 0 1-14.271 0" ></ path > < path d = "m21 21-2.16-3.84" ></ path > < path d = "m3 21 8.02-14.26" ></ path > < circle cy = "5" r = "2" cx = "12" ></ circle > < / > } } pub const LUCIDE_DRAFTING_COMPASS : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("width" , "24") , ("fill" , "none") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("stroke-width" , "2")] } ;