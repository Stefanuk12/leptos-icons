use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cx = "12" cy = "5" r = "2" ></ circle > < path d = "m3 21 8.02-14.26" ></ path > < path d = "m12.99 6.74 1.93 3.44" ></ path > < path d = "M19 12c-3.87 4-10.13 4-14 0" ></ path > < path d = "m21 21-2.16-3.84" ></ path > < / > } } pub const LUCIDE_DRAFTING_COMPASS : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("stroke-width" , "2") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor")] } ;