use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m7 7 10 10-5 5V2l5 5L7 17" ></ path > < line x2 = "21" x1 = "18" y2 = "12" y1 = "12" ></ line > < line x2 = "6" y1 = "12" y2 = "12" x1 = "3" ></ line > < / > } } pub const LUCIDE_BLUETOOTH_CONNECTED : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("height" , "24") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round")] } ;