use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m7 7 10 10-5 5V2l5 5L7 17" ></ path > < line y1 = "12" x2 = "21" y2 = "12" x1 = "18" ></ line > < line x1 = "3" y2 = "12" x2 = "6" y1 = "12" ></ line > < / > } } pub const LucideBluetoothConnected : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("width" , "24") , ("stroke-width" , "2")] } ;