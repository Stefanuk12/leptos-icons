use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect rx = "2" ry = "2" height = "18" width = "18" x = "3" y = "3" ></ rect > < polyline points = "11 3 11 11 14 8 17 11 17 3" ></ polyline > < / > } } pub const LUCIDE_ALBUM : Path = Path { path : icon_path , props : & [("height" , "24") , ("stroke-width" , "2") , ("width" , "24") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor")] } ;