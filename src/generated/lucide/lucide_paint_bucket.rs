use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m19 11-8-8-8.6 8.6a2 2 0 0 0 0 2.8l5.2 5.2c.8.8 2 .8 2.8 0L19 11Z" ></ path > < path d = "m5 2 5 5" ></ path > < path d = "M2 13h15" ></ path > < path d = "M22 20a2 2 0 1 1-4 0c0-1.6 1.7-2.4 2-4 .3 1.6 2 2.4 2 4Z" ></ path > < / > } } pub const LUCIDE_PAINT_BUCKET : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("width" , "24") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;