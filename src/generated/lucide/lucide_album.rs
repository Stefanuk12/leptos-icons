use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect ry = "2" x = "3" width = "18" height = "18" rx = "2" y = "3" ></ rect > < polyline points = "11 3 11 11 14 8 17 11 17 3" ></ polyline > < / > } } pub const LUCIDE_ALBUM : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("stroke-linecap" , "round") , ("height" , "24") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor")] } ;