use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect height = "15" rx = "2" width = "20" ry = "2" x = "2" y = "7" ></ rect > < polyline points = "17 2 12 7 7 2" ></ polyline > < / > } } pub const LUCIDE_TV : Path = Path { path : icon_path , props : & [("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("stroke-linecap" , "round") , ("stroke" , "currentColor")] } ;