use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m7 7 10 10-5 5V2l5 5L7 17" ></ path > < line y2 = "12" x2 = "21" y1 = "12" x1 = "18" ></ line > < line x1 = "3" y2 = "12" y1 = "12" x2 = "6" ></ line > < / > } } pub const LUCIDE_BLUETOOTH_CONNECTED : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("width" , "24") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none")] } ;