use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m7 7 10 10-5 5V2l5 5L7 17" ></ path > < line x1 = "18" y2 = "12" x2 = "21" y1 = "12" ></ line > < line y1 = "12" y2 = "12" x1 = "3" x2 = "6" ></ line > < / > } } pub const LUCIDE_BLUETOOTH_CONNECTED : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("fill" , "none") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor")] } ;