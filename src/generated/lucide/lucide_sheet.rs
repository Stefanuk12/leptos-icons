use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect ry = "2" x = "3" y = "3" height = "18" width = "18" rx = "2" ></ rect > < line x1 = "3" x2 = "21" y1 = "9" y2 = "9" ></ line > < line y1 = "15" x2 = "21" y2 = "15" x1 = "3" ></ line > < line x2 = "9" y1 = "9" x1 = "9" y2 = "21" ></ line > < line y2 = "21" y1 = "9" x2 = "15" x1 = "15" ></ line > < / > } } pub const LUCIDE_SHEET : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("height" , "24") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;