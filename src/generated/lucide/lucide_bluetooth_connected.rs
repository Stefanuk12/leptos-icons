use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m7 7 10 10-5 5V2l5 5L7 17" ></ path > < line x1 = "18" x2 = "21" y2 = "12" y1 = "12" ></ line > < line x2 = "6" y2 = "12" x1 = "3" y1 = "12" ></ line > < / > } } pub const LUCIDE_BLUETOOTH_CONNECTED : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("width" , "24") , ("height" , "24") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24")] } ;