use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M12 2v5" ></ path > < path d = "M6 7h12l4 9H2l4-9Z" ></ path > < path d = "M9.17 16a3 3 0 1 0 5.66 0" ></ path > < / > } } pub const LUCIDE_LAMP_CEILING : Path = Path { path : icon_path , props : & [("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("width" , "24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("fill" , "none")] } ;