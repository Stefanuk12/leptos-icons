use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M15 3h4a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2h-4" ></ path > < polyline points = "10 17 15 12 10 7" ></ polyline > < line x1 = "15" y2 = "12" y1 = "12" x2 = "3" ></ line > < / > } } pub const LUCIDE_LOG_IN : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("width" , "24") , ("stroke-width" , "2") , ("height" , "24") , ("stroke" , "currentColor")] } ;