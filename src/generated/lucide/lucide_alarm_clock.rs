use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cx = "12" r = "8" cy = "13" ></ circle > < path d = "M12 9v4l2 2" ></ path > < path d = "M5 3 2 6" ></ path > < path d = "m22 6-3-3" ></ path > < path d = "M6.38 18.7 4 21" ></ path > < path d = "M17.64 18.67 20 21" ></ path > < / > } } pub const LUCIDE_ALARM_CLOCK : Path = Path { path : icon_path , props : & [("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("height" , "24")] } ;