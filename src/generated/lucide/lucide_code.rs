use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < polyline points = "16 18 22 12 16 6" ></ polyline > < polyline points = "8 6 2 12 8 18" ></ polyline > < / > } } pub const LUCIDE_CODE : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("width" , "24") , ("fill" , "none") , ("stroke-width" , "2") , ("stroke-linejoin" , "round")] } ;