use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x1 = "2" y1 = "2" y2 = "22" x2 = "22" ></ line > < line y1 = "17" x1 = "12" y2 = "22" x2 = "12" ></ line > < path d = "M9 9v1.76a2 2 0 0 1-1.11 1.79l-1.78.9A2 2 0 0 0 5 15.24V17h12" ></ path > < path d = "M15 9.34V6h1a2 2 0 0 0 0-4H7.89" ></ path > < / > } } pub const LucidePinOff : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("height" , "24") , ("width" , "24") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-width" , "2") , ("stroke-linecap" , "round")] } ;