use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cx = "12" r = "8" cy = "13" ></ circle > < path d = "M5 3 2 6" ></ path > < path d = "m22 6-3-3" ></ path > < path d = "M6.38 18.7 4 21" ></ path > < path d = "M17.64 18.67 20 21" ></ path > < path d = "M12 10v6" ></ path > < path d = "M9 13h6" ></ path > < / > } } pub const LUCIDE_ALARM_CLOCK_PLUS : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("height" , "24") , ("width" , "24") , ("stroke-width" , "2")] } ;