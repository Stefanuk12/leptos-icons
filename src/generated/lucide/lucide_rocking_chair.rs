use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < polyline points = "3.5 2 6.5 12.5 18 12.5" ></ polyline > < line y1 = "12.5" x2 = "5.5" y2 = "20" x1 = "9.5" ></ line > < line x1 = "15" y1 = "12.5" x2 = "18.5" y2 = "20" ></ line > < path d = "M2.75 18a13 13 0 0 0 18.5 0" ></ path > < / > } } pub const LUCIDE_ROCKING_CHAIR : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("stroke-linecap" , "round") , ("height" , "24") , ("width" , "24") , ("fill" , "none") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round")] } ;