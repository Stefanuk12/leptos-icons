use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect ry = "2" rx = "2" width = "18" x = "3" height = "18" y = "3" ></ rect > < line y2 = "9" x2 = "21" y1 = "9" x1 = "3" ></ line > < line x2 = "21" y1 = "15" y2 = "15" x1 = "3" ></ line > < line y2 = "21" y1 = "9" x1 = "9" x2 = "9" ></ line > < line x2 = "15" x1 = "15" y1 = "9" y2 = "21" ></ line > < / > } } pub const LUCIDE_SHEET : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("fill" , "none") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("height" , "24")] } ;