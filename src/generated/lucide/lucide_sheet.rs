use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "18" y = "3" rx = "2" x = "3" height = "18" ry = "2" ></ rect > < line x2 = "21" y2 = "9" x1 = "3" y1 = "9" ></ line > < line x1 = "3" x2 = "21" y1 = "15" y2 = "15" ></ line > < line y2 = "21" y1 = "9" x1 = "9" x2 = "9" ></ line > < line x1 = "15" y1 = "9" y2 = "21" x2 = "15" ></ line > < / > } } pub const LUCIDE_SHEET : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("width" , "24")] } ;