use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M16 16h6" ></ path > < path d = "M21 10V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l2-1.14" ></ path > < path d = "m7.5 4.27 9 5.15" ></ path > < polyline points = "3.29 7 12 12 20.71 7" ></ polyline > < line y1 = "22" x2 = "12" y2 = "12" x1 = "12" ></ line > < / > } } pub const LUCIDE_PACKAGE_MINUS : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("width" , "24")] } ;