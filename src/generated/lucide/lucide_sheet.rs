use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect ry = "2" height = "18" x = "3" width = "18" y = "3" rx = "2" ></ rect > < line x1 = "3" x2 = "21" y1 = "9" y2 = "9" ></ line > < line x1 = "3" x2 = "21" y1 = "15" y2 = "15" ></ line > < line x2 = "9" x1 = "9" y2 = "21" y1 = "9" ></ line > < line x2 = "15" y1 = "9" y2 = "21" x1 = "15" ></ line > < / > } } pub const LUCIDE_SHEET : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("width" , "24") , ("height" , "24") , ("viewBox" , "0 0 24 24")] } ;