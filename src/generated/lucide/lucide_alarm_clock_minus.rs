use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M5 3 2 6" ></ path > < path d = "m22 6-3-3" ></ path > < path d = "M6.38 18.7 4 21" ></ path > < path d = "M17.64 18.67 20 21" ></ path > < path d = "M9 13h6" ></ path > < / > } } pub const LucideAlarmClockMinus : Path = Path { path : icon_path , props : & [("fill" , "none") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor")] } ;