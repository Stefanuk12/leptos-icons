use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m17 17-5 5V12l-5 5" ></ path > < path d = "m2 2 20 20" ></ path > < path d = "M14.5 9.5 17 7l-5-5v4.5" ></ path > < / > } } pub const LUCIDE_BLUETOOTH_OFF : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("height" , "24") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke-width" , "2")] } ;