use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M7 18v-6a5 5 0 1 1 10 0v6" ></ path > < path d = "M5 21a1 1 0 0 0 1 1h12a1 1 0 0 0 1-1v-1a2 2 0 0 0-2-2H7a2 2 0 0 0-2 2z" ></ path > < path d = "M21 12h1" ></ path > < path d = "M18.5 4.5 18 5" ></ path > < path d = "M2 12h1" ></ path > < path d = "M12 2v1" ></ path > < path d = "m4.929 4.929.707.707" ></ path > < path d = "M12 12v6" ></ path > < / > } } pub const LUCIDE_SIREN : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("width" , "24") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("height" , "24")] } ;