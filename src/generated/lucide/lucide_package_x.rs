use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M21 10V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l2-1.14" ></ path > < path d = "m7.5 4.27 9 5.15" ></ path > < polyline points = "3.29 7 12 12 20.71 7" ></ polyline > < line x2 = "12" x1 = "12" y2 = "12" y1 = "22" ></ line > < path d = "m17 13 5 5m-5 0 5-5" ></ path > < / > } } pub const LUCIDE_PACKAGE_X : Path = Path { path : icon_path , props : & [("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke-linejoin" , "round") , ("width" , "24") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24")] } ;