use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cx = "12" cy = "13" r = "8" ></ circle > < path d = "M5 3 2 6" ></ path > < path d = "m22 6-3-3" ></ path > < path d = "M6.38 18.7 4 21" ></ path > < path d = "M17.64 18.67 20 21" ></ path > < path d = "m9 13 2 2 4-4" ></ path > < / > } } pub const LUCIDE_ALARM_CLOCK_CHECK : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("width" , "24") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor")] } ;