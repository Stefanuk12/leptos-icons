use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < polyline points = "16 18 22 12 16 6" ></ polyline > < polyline points = "8 6 2 12 8 18" ></ polyline > < / > } } pub const LUCIDE_CODE : Path = Path { path : icon_path , props : & [("height" , "24") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("fill" , "none") , ("width" , "24") , ("viewBox" , "0 0 24 24")] } ;