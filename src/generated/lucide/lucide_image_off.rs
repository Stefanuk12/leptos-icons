use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x1 = "2" x2 = "22" y1 = "2" y2 = "22" ></ line > < path d = "M10.41 10.41a2 2 0 1 1-2.83-2.83" ></ path > < line x1 = "13.5" y2 = "21" y1 = "13.5" x2 = "6" ></ line > < line x2 = "21" y1 = "12" y2 = "15" x1 = "18" ></ line > < path d = "M3.59 3.59A1.99 1.99 0 0 0 3 5v14a2 2 0 0 0 2 2h14c.55 0 1.052-.22 1.41-.59" ></ path > < path d = "M21 15V5a2 2 0 0 0-2-2H9" ></ path > < / > } } pub const LUCIDE_IMAGE_OFF : Path = Path { path : icon_path , props : & [("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("fill" , "none") , ("stroke" , "currentColor") , ("width" , "24") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round")] } ;