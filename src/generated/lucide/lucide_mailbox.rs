use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M22 17a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V9.5C2 7 4 5 6.5 5H18c2.2 0 4 1.8 4 4v8Z" ></ path > < polyline points = "15,9 18,9 18,11" ></ polyline > < path d = "M6.5 5C9 5 11 7 11 9.5V17a2 2 0 0 1-2 2" ></ path > < line x1 = "6" y2 = "10" y1 = "10" x2 = "7" ></ line > < / > } } pub const LUCIDE_MAILBOX : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke-width" , "2") , ("width" , "24") , ("stroke-linecap" , "round") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24")] } ;