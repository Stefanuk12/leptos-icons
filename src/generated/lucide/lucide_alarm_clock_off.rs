use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M6.87 6.87a8 8 0 1 0 11.26 11.26" ></ path > < path d = "M19.9 14.25a8 8 0 0 0-9.15-9.15" ></ path > < path d = "m22 6-3-3" ></ path > < path d = "M6.26 18.67 4 21" ></ path > < path d = "m2 2 20 20" ></ path > < path d = "M4 4 2 6" ></ path > < / > } } pub const LUCIDE_ALARM_CLOCK_OFF : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("height" , "24") , ("width" , "24") , ("fill" , "none")] } ;