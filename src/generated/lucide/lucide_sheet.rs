use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect ry = "2" rx = "2" height = "18" width = "18" x = "3" y = "3" ></ rect > < line y1 = "9" x2 = "21" x1 = "3" y2 = "9" ></ line > < line x1 = "3" y2 = "15" x2 = "21" y1 = "15" ></ line > < line x1 = "9" y2 = "21" x2 = "9" y1 = "9" ></ line > < line x1 = "15" x2 = "15" y1 = "9" y2 = "21" ></ line > < / > } } pub const LUCIDE_SHEET : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("height" , "24") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("width" , "24") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24")] } ;