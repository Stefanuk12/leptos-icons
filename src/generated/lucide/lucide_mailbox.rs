use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M22 17a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V9.5C2 7 4 5 6.5 5H18c2.2 0 4 1.8 4 4v8Z" ></ path > < polyline points = "15,9 18,9 18,11" ></ polyline > < path d = "M6.5 5C9 5 11 7 11 9.5V17a2 2 0 0 1-2 2" ></ path > < line y2 = "10" x2 = "7" x1 = "6" y1 = "10" ></ line > < / > } } pub const LUCIDE_MAILBOX : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("width" , "24") , ("height" , "24") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("stroke" , "currentColor")] } ;