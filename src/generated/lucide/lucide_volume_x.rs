use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M11 4.702a.705.705 0 0 0-1.203-.498L6.413 7.587A1.4 1.4 0 0 1 5.416 8H3a1 1 0 0 0-1 1v6a1 1 0 0 0 1 1h2.416a1.4 1.4 0 0 1 .997.413l3.383 3.384A.705.705 0 0 0 11 19.298z" ></ path > < line x1 = "22" x2 = "16" y1 = "9" y2 = "15" ></ line > < line x2 = "22" y1 = "9" y2 = "15" x1 = "16" ></ line > < / > } } pub const LUCIDE_VOLUME_X : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke-linejoin" , "round") , ("height" , "24") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round")] } ;