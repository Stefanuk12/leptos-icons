use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M15 3h4a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2h-4" ></ path > < polyline points = "10 17 15 12 10 7" ></ polyline > < line y1 = "12" y2 = "12" x1 = "15" x2 = "3" ></ line > < / > } } pub const LucideLogIn : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("height" , "24")] } ;