use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect rx = "2" width = "20" ry = "2" x = "2" y = "7" height = "15" ></ rect > < polyline points = "17 2 12 7 7 2" ></ polyline > < / > } } pub const LUCIDE_TV : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("fill" , "none") , ("width" , "24") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24")] } ;