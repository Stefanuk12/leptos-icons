use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M10 10 4.72 20.55a1 1 0 0 0 .9 1.45h12.76a1 1 0 0 0 .9-1.45l-1.272-2.542" ></ path > < path d = "M10 2v2.343" ></ path > < path d = "M14 2v6.343" ></ path > < path d = "M8.5 2h7" ></ path > < path d = "M7 16h9" ></ path > < line y1 = "2" y2 = "22" x1 = "2" x2 = "22" ></ line > < / > } } pub const LUCIDE_FLASK_CONICAL_OFF : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke-width" , "2") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("height" , "24")] } ;