use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m5.2 6.2 1.4 1.4" ></ path > < path d = "M2 13h2" ></ path > < path d = "M20 13h2" ></ path > < path d = "m17.4 7.6 1.4-1.4" ></ path > < path d = "M22 17H2" ></ path > < path d = "M22 21H2" ></ path > < path d = "M16 13a4 4 0 0 0-8 0" ></ path > < path d = "M12 5V2.5" ></ path > < / > } } pub const LUCIDE_HAZE : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("height" , "24") , ("width" , "24") , ("stroke-linecap" , "round")] } ;