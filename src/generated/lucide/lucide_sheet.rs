use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect y = "3" x = "3" rx = "2" height = "18" ry = "2" width = "18" ></ rect > < line y2 = "9" y1 = "9" x2 = "21" x1 = "3" ></ line > < line x2 = "21" y1 = "15" x1 = "3" y2 = "15" ></ line > < line x2 = "9" x1 = "9" y2 = "21" y1 = "9" ></ line > < line x1 = "15" y2 = "21" x2 = "15" y1 = "9" ></ line > < / > } } pub const LUCIDE_SHEET : Path = Path { path : icon_path , props : & [("fill" , "none") , ("height" , "24") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("width" , "24")] } ;