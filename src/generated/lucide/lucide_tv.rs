use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect height = "15" x = "2" ry = "2" y = "7" width = "20" rx = "2" ></ rect > < polyline points = "17 2 12 7 7 2" ></ polyline > < / > } } pub const LUCIDE_TV : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("fill" , "none") , ("width" , "24") , ("stroke-width" , "2") , ("height" , "24") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24")] } ;