use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < polygon points = "19 20 9 12 19 4 19 20" ></ polygon > < line y2 = "5" x1 = "5" x2 = "5" y1 = "19" ></ line > < / > } } pub const LUCIDE_SKIP_BACK : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("stroke" , "currentColor") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke-linecap" , "round")] } ;