use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m7 7 10 10-5 5V2l5 5L7 17" ></ path > < line y2 = "12" x2 = "21" y1 = "12" x1 = "18" ></ line > < line x1 = "3" x2 = "6" y1 = "12" y2 = "12" ></ line > < / > } } pub const LUCIDE_BLUETOOTH_CONNECTED : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("width" , "24") , ("fill" , "none") , ("height" , "24") , ("stroke-linecap" , "round") , ("stroke-width" , "2")] } ;