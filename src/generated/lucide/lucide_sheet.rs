use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect ry = "2" height = "18" x = "3" y = "3" rx = "2" width = "18" ></ rect > < line y1 = "9" y2 = "9" x1 = "3" x2 = "21" ></ line > < line y1 = "15" y2 = "15" x1 = "3" x2 = "21" ></ line > < line y2 = "21" x1 = "9" y1 = "9" x2 = "9" ></ line > < line x2 = "15" x1 = "15" y1 = "9" y2 = "21" ></ line > < / > } } pub const LUCIDE_SHEET : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("stroke-width" , "2") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("height" , "24")] } ;